import { useCallback, useEffect, useRef, useState } from "react";
import { useParams } from "react-router";

import Delete from "./Actions/Delete";

import EditIcon from "../../assets/Icons/Edit";

import "./Dropdown.scss";

function Dropdown() {
  const dropdownRef = useRef(null);
  const params = useParams();

  const [dropdownVisible, setDropdownVisible] = useState(false);

  const handleClick = useCallback((e) => {
    if (!dropdownRef.current) return;

    if (!dropdownRef.current.contains(e.target)) {
      setDropdownVisible(false);
    }
  }, []);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  const handleToggle = useCallback(() => {
    setDropdownVisible(!dropdownVisible);
  }, [dropdownVisible]);

  return (
    <div className="dropdown" ref={dropdownRef}>
      <div
        className={`toggle visible-${dropdownVisible}`}
        onClick={handleToggle}
      >
        <div />
        <div />
        <div />
      </div>
      <div className={`dropDownContent visible-${dropdownVisible}`}>
        <Delete id={params.id} />
        <button className="rename">
          Rename library
          <EditIcon />
        </button>
      </div>
    </div>
  );
}

export default Dropdown;
