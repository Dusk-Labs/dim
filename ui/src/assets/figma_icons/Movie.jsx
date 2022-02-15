import React from "react";

function Movie(props) {
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
          d="M2 2v20"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path d="M2 12h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path
          d="M18 12h4"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M6 12h12"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path d="M2 8h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path d="M2 4h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path d="M18 8h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path d="M18 4h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path
          d="M18 20h4"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M18 16h4"
          stroke={fill}
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path d="M2 20h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path d="M2 16h4" stroke={fill} strokeMiterlimit="10" strokeWidth="2" />
        <path
          d="M22 2v20"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
        <path
          d="M18 2H6v20h12V2z"
          stroke={fill}
          strokeLinecap="square"
          strokeMiterlimit="10"
          strokeWidth="2"
        />
      </g>
    </svg>
  );
}

export default Movie;
