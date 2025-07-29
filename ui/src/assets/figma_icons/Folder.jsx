import React from "react";

function Folder(props) {
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
          d="M21 22H3a2 2 0 0 1-2-2V2h8l3 4h11v14a2 2 0 0 1-2 2z"
          fill={fill}
        />
      </g>
    </svg>
  );
}

export default Folder;
