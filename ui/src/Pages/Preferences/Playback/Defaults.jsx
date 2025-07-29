import { useCallback, useState, useRef, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";

import "./Defaults.scss";

function Defaults() {
  const dropdownRef = useRef(null);
  const dispatch = useDispatch();
  const settings = useSelector((store) => store.settings);

  const [dropdownVisible, setDropdownVisible] = useState(false);

  const setVideoQuality = useCallback(
    (quality) => {
      dispatch(
        updateUserSettings({
          default_video_quality: quality,
        })
      );

      setDropdownVisible(false);
    },
    [dispatch]
  );

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

  const default_quality = settings.userSettings.data.default_video_quality;
  const isDirectPlay = default_quality === "directplay";

  const availableQualities = [
    [1080, 10_000_000],
    [720, 5_000_000],
    [480, 1_000_000],
  ].filter(([resolution, brate]) => {
    if (isDirectPlay) return true;

    const [selected_res, selected_brate] = default_quality.resolution;
    const isSelected = selected_res === resolution && selected_brate === brate;

    return !isSelected;
  });

  let CS = "Direct Play";

  if (!isDirectPlay) {
    const norm_brate = default_quality.resolution[1] / 1_000_000;
    CS = `${default_quality.resolution[0]}p - ${norm_brate}MB`;
  }

  return (
    <section className="preferencesPlaybackDefaults">
      <h2>Player settings</h2>
      <div className="field">
        <p>Select default video quality</p>
        <div className="dropdown" ref={dropdownRef}>
          <div
            className={`toggle visible-${dropdownVisible}`}
            onClick={() => setDropdownVisible(!dropdownVisible)}
          >
            <p>{CS}</p>
          </div>
          <div className={`dropDownContent visible-${dropdownVisible}`}>
            {!isDirectPlay && (
              <button onClick={() => setVideoQuality("directplay")}>
                Direct Play
              </button>
            )}
            {availableQualities.map(([resolution, brate], i) => (
              <button
                key={i}
                onClick={() =>
                  setVideoQuality({ resolution: [resolution, brate] })
                }
              >
                {`${resolution}p - ${brate / 1_000_000}MB`}
              </button>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}

export default Defaults;
