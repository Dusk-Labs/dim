import { useCallback, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { useParams } from "react-router";

import Delete from "./Actions/Delete";

import EditIcon from "../../assets/Icons/Edit";

import "./Dropdown.scss";

function Dropdown() {
  const dropdownRef = useRef(null);
  const params = useParams();

  const user = useSelector((store) => store.user);

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
      {user.info.roles?.includes("owner") && (
        <div className={`dropDownContent visible-${dropdownVisible}`}>
          <Delete id={params.id} />
          <button className="rename">
            Rename library
            <EditIcon />
          </button>
        </div>
      )}
    </div>
  );
}

export default Dropdown;
