import React from "react";

function Ignore(props) {
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
          d="M5.05 14.95a7 7 0 1 1 9.9-9.9"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M16.71 8A7.004 7.004 0 0 1 8 16.71"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M14.95 14.95L21 21"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M18 2L2 18"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
      </g>
    </svg>
  );
}

export default Ignore;
