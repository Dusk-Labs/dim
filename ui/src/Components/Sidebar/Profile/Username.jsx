import { useCallback, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";

import LogoutBtn from "./LogoutBtn";
import CaretDown from "../../../assets/Icons/CaretDown";

import "./Username.scss";

function Username(_props) {
  const user = useSelector((store) => store?.user || {});

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

  if (!user.fetched || user.error) return <></>;

  return (
    <div className="quickLinksDropdown" ref={dropdownRef}>
      <div className="username">
        <p onClick={handleToggle}>{user.info.username || "eray_chumak"}</p>
        <div
          className={`toggle visible-${dropdownVisible}`}
          onClick={handleToggle}
        >
          <CaretDown />
        </div>
      </div>
      <div className={`dropDownContent visible-${dropdownVisible}`}>
        <LogoutBtn />
      </div>
    </div>
  );
}

export default Username;
