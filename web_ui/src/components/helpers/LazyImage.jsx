import React from "react";

function LazyImage(props) {
    const { data, alt } = props;

    return data !== undefined
        ? <div className="image-wrapper"><img src={data} alt={alt}></img></div>
        : <div className="placeholder"/>;
}

export default LazyImage;
