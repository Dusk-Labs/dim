import { useCallback, useState, useRef, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";

import "./Defaults.scss";

function Defaults() {
  const dropdownRef = useRef(null);
  const dispatch = useDispatch();
  const settings = useSelector(store => store.settings);

  const [dropdownVisible, setDropdownVisible] = useState(false);

  const handleToggle = useCallback(() => {
    if (!dropdownVisible) {
      setDropdownVisible(true);
    } else {
      setDropdownVisible(false);
    }
  }, [dropdownVisible]);

  const setVideoQuality = useCallback(quality => {
    dispatch(updateUserSettings({
      "default_video_quality": quality === "directplay" ? quality : {
        "resolution": quality
      }
    }));

    setDropdownVisible(false);
  }, [dispatch]);

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

  const availableQualities = [[1080, 10_000_000], [720, 5_000_000], [480, 1_000_000]];
  const default_quality = settings.userSettings.data.default_video_quality;
  const isDirectPlay = default_quality === "directplay";

  const currentlySelected = availableQualities.filter(([resolution, brate]) => {
    const [selected_res, selected_brate] = (
      isDirectPlay ? [null, null] : default_quality["resolution"]
    );

    const isSelected = selected_res === resolution && selected_brate === brate;

    return isSelected;
  });

  const norm_brate = currentlySelected[0][1] / 1_000_000;
  const CS = `${currentlySelected[0][0]}p - ${norm_brate}MB`;

  return (
    <section className="preferencesPlaybackDefaults">
      <h2>Player settings</h2>
      <div className="field">
        <p>Select default video quality</p>
        <div className="dropdown" ref={dropdownRef}>
          <div
            className={`toggle visible-${dropdownVisible}`}
            onClick={handleToggle}
          >
            <p>{CS}</p>
          </div>
          <div className={`dropDownContent visible-${dropdownVisible}`}>
            {isDirectPlay && (
              <button>Direct Play</button>
            )}
            {availableQualities.map(([resolution, brate], i) => {
              const norm_brate = brate / 1_000_000;
              const label = `${resolution}p - ${norm_brate}MB`;

              const [selected_res, selected_brate] = isDirectPlay ? [null, null] : default_quality["resolution"];
              const isSelected = selected_res === resolution && selected_brate === brate;

              if (!isSelected) {
                return (
                  <button
                    key={i}
                    onClick={() => setVideoQuality([resolution, brate])}
                  >
                    {label}
                  </button>
                );
              }
            })}
          </div>
        </div>
      </div>
    </section>
  );
}

export default Defaults;
