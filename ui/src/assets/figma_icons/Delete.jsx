import React from "react";

function Delete(props) {
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
          d="M20 9v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V9"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M1 5h22"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M12 12v6"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M8 12v6"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M16 12v6"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M8 5V1h8v4"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
      </g>
    </svg>
  );
}

export default Delete;
