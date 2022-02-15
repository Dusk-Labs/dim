import React from "react";

function Search(props) {
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
        <g clipPath="url(#clip0_59_7413)">
          <path
            d="M13.358 18.153a9.063 9.063 0 1 0-7.09-16.682 9.063 9.063 0 0 0 7.09 16.682z"
            stroke={fill}
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="1.5"
          />
          <path
            d="M16.221 16.22l7.029 7.03"
            stroke={fill}
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="1.5"
          />
        </g>
        <defs>
          <clipPath id="clip0_59_7413">
            <path d="M0 0h24v24H0z" fill={fill} />
          </clipPath>
        </defs>
      </g>
    </svg>
  );
}

export default Search;
