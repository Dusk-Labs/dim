import React, { useState, useEffect } from "react";
import * as Vibrant from "node-vibrant";

import DimLogo from "../../assets/DimLogo";

function CardImage(props) {
  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    const img = new Image();

    img.onload = async (e) => {
      setLoaded(true);

      try {
        const color = await Vibrant.from(e.target).getPalette();

        props.setBG(color.Vibrant.getHex());
        props.setText(color.Vibrant.getTitleTextColor());
      } catch (e) {}
    };

    img.onerror = () => {
      setLoaded(true);
      setErr(true);
    };

    img.src = props.src;
  }, [props.src]);

  return (
    <div className="cardImageWrapper">
      {(loaded && !error) && (
        <img src={props.src} alt="cover"/>
      )}
      {error && (
        <div className="placeholder">
          <DimLogo/>
        </div>
      )}
    </div>
  )
}

export default CardImage;
