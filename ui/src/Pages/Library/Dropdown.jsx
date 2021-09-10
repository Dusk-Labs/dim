import { useCallback, useContext, useEffect, useRef, useState } from "react";
import { useDispatch } from "react-redux";
import { useParams } from "react-router";

import Delete from "./Actions/Delete";

import EditIcon from "../../assets/Icons/Edit";
import FileVideoIcon from "../../assets/Icons/Wrench";
import { LibraryContext } from "./Context";
import { fetchLibraryUnmatched } from "../../actions/library";

import "./Dropdown.scss";

function Dropdown() {
  const dispatch = useDispatch();

  const { setShowUnmatched, unmatched } = useContext(LibraryContext);

  const dropdownRef = useRef(null);
  const params = useParams();

  const [dropdownVisible, setDropdownVisible] = useState(false);

  const handleClick = useCallback((e) => {
    if (!dropdownRef.current) return;

    if (!dropdownRef.current.contains(e.target)) {
      setDropdownVisible(false);
    }
  }, []);

  const manageUnmatchedMedia = useCallback(() => {
    if (Object.keys(unmatched.items).length === 0) return;

    setShowUnmatched(true);
    setDropdownVisible(false);
  }, [setShowUnmatched, unmatched.items]);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  const handleToggle = useCallback(() => {
    if (!dropdownVisible) {
      setDropdownVisible(true);
      dispatch(fetchLibraryUnmatched(params.id));
    } else {
      setDropdownVisible(false);
    }
  }, [dispatch, dropdownVisible, params.id]);

  const count = Object.values(unmatched.items).flat().length;

  return (
    <div className="dropdown" ref={dropdownRef}>
      <div
        className={`toggle visible-${dropdownVisible}`}
        onClick={handleToggle}
      >
        <div/>
        <div/>
        <div/>
      </div>
      <div className={`dropDownContent visible-${dropdownVisible}`}>
        <Delete id={params.id}/>
        <button className="rename">
          Rename library
          <EditIcon/>
        </button>
        {count > 0 && (
          <button
            onClick={manageUnmatchedMedia}
            disabled={count === 0}
          >
            Match {count} files
            <FileVideoIcon/>
          </button>
        )}
      </div>
    </div>
  );
}

export default Dropdown;
