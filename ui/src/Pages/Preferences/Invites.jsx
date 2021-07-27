import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { formatHHMMSSDate } from "../../Helpers/utils";

import { fetchInvites, createNewInvite } from "../../actions/auth.js";

import "./Invites.scss";

function Invites() {
  const dispatch = useDispatch();

  const auth = useSelector(store => store.auth);

  useEffect(() => {
    console.log(auth);
  }, [auth]);

  useEffect(() => {
    dispatch(fetchInvites());
  }, [auth.admin_exists, dispatch]);

  return (
    <div className="preferencesInvites">
      <section>
        <h2>Manage invites</h2>
        <p className="desc">Create a token to invite someone and allow them access to your added media libraries securely.</p>
        <h3>Tokens</h3>
        <div className="tokensContainer">
          <div className="heading">
            <p>ID</p>
            <p>Created at</p>
            <p>Status</p>
          </div>
          <div className="separator"/>
          <div className="tokens">
            {auth.invites.items.map((token, i) => {
              const {hours, mins, secs, date, month, year} = formatHHMMSSDate(token.created);

              return (
                <div className="token" key={i}>
                  <p>{token.id}</p>
                  <p>{hours}:{mins}:{secs} on the {date}/{month}/{year}</p>
                  {token.claimed_by
                    ? <p>Claimed by {token.claimed_by}</p>
                    : <p>Available</p>
                  }
                </div>
              );
            })}
          </div>
        </div>
        <button>
          Generate a new token
        </button>
      </section>
    </div>
  );
}

export default Invites;
