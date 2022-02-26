/// Source: https://github.com/seanmonstar/warp/issues/619
/// Takes a list of handler expressions and `or`s them together
/// in a balanced tree. That is, instead of `a.or(b).or(c).or(d)`,
/// it produces `(a.or(b)).or(c.or(d))`, thus nesting the types
/// less deeply, which provides improvements in compile time.
///
/// It also applies `::warp::Filter::boxed` to each handler expression
/// when in `debug_assertions` mode, improving compile time further.
///
//
// The basic list splitting algorithm here is based on this gist:
// https://gist.github.com/durka/9fc479de2555225a787f
// It uses a counter from which two items are removed each time,
// stopping when the counter reaches 0. At each step, one item
// is moved from the left to the right, and thus at the end,
// there will be the same number of items in each list.
//
// The flow is as follows:
// - If there is one handler expression, debug_box it and return.
// - If there is more than one handler expression:
//   - First, copy the list into two: the one that will go into the
//     right side of the `or`, and one that will serve as a counter.
//     Recurse with these separated by semicolons, plus an empty `left`
//     list before the first semicolon.
//   - Then, as long as there are at least two items in the counter
//     list, remove them and move the first item on the right side of
//     the first semicolon (`head`) to the left side of the first semicolon.
//   - Finally, when there are one or zero items left in the counter,
//     move one last item to the left, make the call this macro on both the
//     left and right sides, and `or` the two sides together.
//
// For example, balanced_or_tree!(a, b, c, d, e) would take the following steps:
//
// - balanced_or_tree!(a, b, c, d, e)
// - balanced_or_tree!(@internal ; a, b, c, d, e ; a, b, c, d, e) // initialise lists
// - balanced_or_tree!(@internal a ; b, c, d, e ; c, d, e) // move one elem; remove two
// - balanced_or_tree!(@internal a, b ; c, d, e ; e) // now only one elem in counter
// - balanced_or_tree!(a, b, c).or(balanced_or_tree(d, e)) // recurse on each sublist
#[macro_export]
macro_rules! balanced_or_tree {
    // Base case: just a single expression, return it wrapped in `debug_boxed`
    ($x:expr $(,)?) => { $x };
    // Multiple expressions: recurse with three lists: left, right and counter.
    ($($x:expr),+ $(,)?) => {
        balanced_or_tree!(@internal  ;     $($x),+; $($x),+)
        //                          ^ left ^ right  ^ counter
    };
    // Counter 1 or 2; move one more item and recurse on each sublist, and or them together
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr $(,$b:expr)?) => {
        (balanced_or_tree!($($left,)* $head)).or(balanced_or_tree!($($tail),+))
    };
    // Counter > 2; move one item from the right to the left and subtract two from the counter
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr, $b:expr, $($more:expr),+) => {
        balanced_or_tree!(@internal $($left,)* $head; $($tail),+; $($more),+)
    };
}

#[macro_export]
macro_rules! warp_try {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(e) => return Ok(::warp::reply::Reply::into_response(e)),
        }
    };
}

#[macro_export]
macro_rules! warp_unwrap {
    ($x:expr) => {
        match $x {
            Ok(x) => {
                Result::<_, ::core::convert::Infallible>::Ok(::warp::reply::Reply::into_response(x))
            }
            Err(e) => {
                Result::<_, ::core::convert::Infallible>::Ok(::warp::reply::Reply::into_response(e))
            }
        }
    };
}

/// Construct a `serde_json::Value` from a JSON literal.
///
/// ```
/// # use serde_json::json;
/// #
/// let value = json!({
///     "code": 200,
///     "success": true,
///     "payload": {
///         "features": [
///             "serde",
///             "json"
///         ]
///     }
/// });
/// ```
///
/// Variables or expressions can be interpolated into the JSON literal. Any type
/// interpolated into an array element or object value must implement Serde's
/// `Serialize` trait, while any type interpolated into a object key must
/// implement `Into<String>`. If the `Serialize` implementation of the
/// interpolated type decides to fail, or if the interpolated type contains a
/// map with non-string keys, the `json!` macro will panic.
///
/// ```
/// # use serde_json::json;
/// #
/// let code = 200;
/// let features = vec!["serde", "json"];
///
/// let value = json!({
///     "code": code,
///     "success": code == 200,
///     "payload": {
///         features[0]: features[1]
///     }
/// });
/// ```
///
/// Trailing commas are allowed inside both arrays and objects.
///
/// ```
/// # use serde_json::json;
/// #
/// let value = json!([
///     "notice",
///     "the",
///     "trailing",
///     "comma -->",
/// ]);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! json {
    // Hide distracting implementation details from the generated rustdoc.
    ($($json:tt)+) => {
        json_internal!($($json)+)
    };
}

// Rocket relies on this because they export their own `json!` with a different
// doc comment than ours, and various Rust bugs prevent them from calling our
// `json!` from their `json!` so they call `json_internal!` directly. Check with
// @SergioBenitez before making breaking changes to this macro.
//
// Changes are fine as long as `json_internal!` does not call any new helper
// macros and can still be invoked as `json_internal!($($json)+)`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: json_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        json_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        json_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        json_internal!(@array [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: json_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        json_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!($value)) , $($rest)*);
    };

    // Next value is an optional expression followed by comma.
    (@object $object:ident ($($key:tt)+) (:? $value:expr , $($rest:tt)*) $copy:tt) => {
        if let Some(x) = $value {
            json_internal!(@object $object [$($key)+] (json_internal!($value)) , $($rest)*);
        } else {
            json_internal!(@object $object () ($($rest)*) ($($rest)*));
        }
    };

    // Next entry is a key with the spread operator and no value
    (@object $object:ident ($(...$key:expr)+) (, $($rest:tt)*) $copy:tt) => {
        if let Some(map) = ($($key)+).clone().as_object_mut() {
            let _ = $object.append(map);
        }

        json_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Last entry is a key with the spread operator and no value.
    (@object $object:ident ($(...$key:expr)+) () $copy:tt) => {
        if let Some(map) = ($($key)+).clone().as_object_mut() {
            let _ = $object.append(map);
        }
    };

    // Next entry is a key with the optional spread operator and no value.
    // This takes in a `Option` and will merge the value if its `Some`.
    (@object $object:ident ($(..?$key:expr)+) (, $($rest:tt)*) $copy:tt) => {
        if let Some(map) = ($($key)+).clone().as_mut().and_then(|x| x.as_object_mut()) {
            let _ = $object.append(map);
        }

        json_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Last entry is a key with the optional spread operator and no value.
    // This takes in a `Option` and will merge the value if its `Some`.
    (@object $object:ident ($(..?$key:expr)+) () $copy:tt) => {
        if let Some(map) = ($($key)+).clone().as_mut().and_then(|x| x.as_object_mut()) {
            let _ = $object.append(map);
        }
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: json_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        ::serde_json::Value::Null
    };

    (true) => {
        ::serde_json::Value::Bool(true)
    };

    (false) => {
        ::serde_json::Value::Bool(false)
    };

    ([]) => {
        ::serde_json::Value::Array(json_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        ::serde_json::Value::Array(json_internal!(@array [] $($tt)+))
    };

    ({}) => {
        ::serde_json::Value::Object(::serde_json::Map::new())
    };

    ({ $($tt:tt)+ }) => {
        ::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            json_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        ::serde_json::to_value(&$other).unwrap()
    };
}

// The json_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

use crate::streaming::Quality;

pub fn quality_to_label(quality: &'static Quality, brate: Option<u64>) -> String {
    let bandwidth_ident = if brate.unwrap_or(quality.bitrate) > 1_000_000 {
        "MB"
    } else {
        "KB"
    };

    let bandwidth_norm = if brate.unwrap_or(quality.bitrate) > 1_000_000 {
        quality.bitrate / 1_000_000
    } else {
        quality.bitrate / 1_000
    };

    format!("{}p@{}{}", quality.height, bandwidth_norm, bandwidth_ident)
}

pub fn ts_to_xml(t: u64) -> String {
    let h = t / 3600;
    let m = t % 3600 / 60;
    let s = t % 3600 % 60;

    let mut tag = "PT".to_string();

    if h != 0 {
        tag = format!("{}{}H", tag, h);
    }

    if m != 0 {
        tag = format!("{}{}M", tag, m);
    }

    if s != 0 {
        tag = format!("{}{}S", tag, s);
    }

    tag
}

pub fn secs_to_pretty(t: u64) -> String {
    let h = t / 3600;
    let m = t % 3600 / 60;

    let mut tag = String::new();

    if h != 0 {
        tag = format!("{h}hr");
    }

    if m != 0 {
        tag = format!("{tag} {m}m");
    }

    tag
}

#[cfg(not(debug_assertions))]
pub fn ffpath(bin: impl AsRef<str>) -> String {
    let mut path = std::env::current_exe().expect("Failed to grab path to the `dim` binary.");
    path.pop(); // remove the dim bin to get the dir of `dim`
    path.push(bin.as_ref());

    path.to_string_lossy().to_string()
}

#[cfg(debug_assertions)]
pub fn ffpath(bin: impl AsRef<str>) -> String {
    bin.as_ref().to_string()
}

pub fn codec_pretty(codec: &str) -> String {
    match codec {
        "h264" => "H.264".into(),
        x => x.to_uppercase(),
    }
}

pub fn channels_pretty(ch: i64) -> String {
    match ch {
        2 | 3 => "2.1".into(),
        6 => "5.1".into(),
        8 => "7.1".into(),
        _ => ch.to_string(),
    }
}

pub fn lang_from_iso639(tag: &str) -> Option<&'static str> {
    dia_i18n::iso_639::LANG_CODES
        .iter()
        .find(|x| x.v2b() == tag)
        .map(|x| x.name())
}
