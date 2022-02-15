import React from "react";

function DottedMenu(props) {
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
        <path d="M14 12a2 2 0 1 0-4 0 2 2 0 0 0 4 0z" fill={fill} />
        <path d="M14 21a2 2 0 1 0-4 0 2 2 0 0 0 4 0z" fill={fill} />
        <path d="M14 3a2 2 0 1 0-4 0 2 2 0 0 0 4 0z" fill={fill} />
      </g>
    </svg>
  );
}

export default DottedMenu;
