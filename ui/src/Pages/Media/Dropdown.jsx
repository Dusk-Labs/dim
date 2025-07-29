import { useCallback, useEffect, useRef, useState } from "react";

import EditIcon from "../../assets/Icons/Edit";

import RematchMediaModal from "../../Modals/RematchMedia/Index";

import "./Dropdown.scss";

function Dropdown() {
  const dropdownRef = useRef(null);
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
    if (!dropdownVisible) {
      setDropdownVisible(true);
    } else {
      setDropdownVisible(false);
    }
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
        <RematchMediaModal>
          <button>
            Rematch
            <EditIcon />
          </button>
        </RematchMediaModal>
      </div>
    </div>
  );
}

export default Dropdown;
