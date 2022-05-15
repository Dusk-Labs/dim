import React from "react";

function Close(props) {
  const fill = props.fill || "currentColor";
  const width = props.width || "100%";
  const height = props.height || "100%";

  return (
    <svg
      height={height}
      width={width}
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <g fill="none">
        <path
          d="M19 5L5 19"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M19 19L5 5"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
      </g>
    </svg>
  );
}

export default Close;
