import React from "react";
import Delete from "./Actions/Delete";

import "./Actions.scss";

const Actions = (props) => {
  return (
    <div className="libraryActions">
      <Delete id={props.id}/>
    </div>
  )
};

export default Actions;