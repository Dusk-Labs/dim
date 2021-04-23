import { useCallback, useEffect, useRef, useState } from "react";
import { useParams } from "react-router";

import Delete from "./Actions/Delete";

import "./Dropdown.scss";

// TODO: move to global Components
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

  return (
    <div className="dropdown" ref={dropdownRef}>
      <div
        className={`toggle visible-${dropdownVisible}`}
        onClick={() => setDropdownVisible(state => !state)}
      >
        <div/>
        <div/>
        <div/>
      </div>
      <div className={`dropDownContent visible-${dropdownVisible}`}>
        <Delete id={params.id}/>
        <button className="rename">
          Rename library
        </button>
        <button className="create">
          Create media from file
        </button>
      </div>
    </div>
  );
}

export default Dropdown;
