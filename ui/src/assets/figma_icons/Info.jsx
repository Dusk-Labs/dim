import React from "react";

function Info(props) {
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
          d="M12 23c6.075 0 11-4.925 11-11S18.075 1 12 1 1 5.925 1 12s4.925 11 11 11z"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M12 11v6"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path d="M12 8a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" fill={fill} />
      </g>
    </svg>
  );
}

export default Info;
