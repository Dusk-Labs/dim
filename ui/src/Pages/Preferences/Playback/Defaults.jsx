import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";
import Button from "../../../Components/Misc/Button";
import Field from "../../Auth/Field";

function Defaults() {
  const dispatch = useDispatch();
  const settings = useSelector(store => store.settings);

  const updateVideoQuality = useCallback((event) => {
    const value = event.target.selectedOptions[0].value;
    const updatedSettings = {
      "default_video_quality": value === "directplay" ? value : {
        "resolution": availableQualities[value]
      }
    };

    dispatch(updateUserSettings(updatedSettings));
  }, [dispatch]);

  const availableQualities = [[1080, 10_000_000], [720, 5_000_000], [480, 1_000_000]];
  const default_quality = settings.userSettings.data.default_video_quality;
  const isDirectPlay = default_quality === "directplay";

  const options = availableQualities.map(([resolution, brate], idx) => {
    const norm_brate = brate / 1_000_000;
    const label = `${resolution}p-${norm_brate}MB`;

    const [selected_res, selected_brate] = isDirectPlay ? [null, null] : default_quality["resolution"];

    const isSelected = selected_res === resolution && selected_brate === brate;
    return isSelected ? <option value={idx} selected>{label}</option> : <option value={idx}>{label}</option>;
  });

  return (
    <section>
      <h2>Player settings</h2>
      <div className="fields">
        <label for="qualities">Default video quality:</label>
        <select id="qualities" name="qualities" onChange={updateVideoQuality}>
          {isDirectPlay ?
            <option value="directplay" selected>Direct Play</option> : <option value="directplay">Direct Play</option>}
          {options}
        </select>
      </div>
    </section>
  );
}

export default Defaults;
