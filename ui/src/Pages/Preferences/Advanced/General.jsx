import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateGlobalSettings } from "../../../actions/settings";
import Button from "../../../Components/Misc/Button";
import Toggle from "../../../Components/Toggle";
import Field from "../../Auth/Field";

function General() {
  const dispatch = useDispatch();
  const settings = useSelector((store) => store.settings);

  const [port, setPort] = useState("");
  const [portErr, setPortErr] = useState("");
  const [verbose, setVerbose] = useState(false);

  useEffect(() => {
    const { data } = settings.globalSettings;

    setPort(data.port);
    setVerbose(data.verbose);
  }, [settings]);

  const toggleVerbose = useCallback(
    (state) => {
      dispatch(
        updateGlobalSettings({
          verbose: state,
        })
      );
    },
    [dispatch]
  );

  const updatePort = useCallback(() => {
    if (port.length === 0 || port < 1 || port > 65535) {
      setPortErr("Invalid port");
      return;
    }

    dispatch(
      updateGlobalSettings({
        port: parseInt(port),
      })
    );
  }, [dispatch, port]);

  const undoUpdatePort = useCallback(() => {
    setPort(settings.globalSettings.data.port);
    setPortErr("");
  }, [settings.globalSettings.data.port]);

  return (
    <section>
      <h2>General</h2>
      <div className="toggles">
        <Toggle
          name="Verbose logging"
          onToggle={toggleVerbose}
          state={verbose}
        />
        <Field
          type="number"
          maxLength="5"
          name="Port"
          data={[port, setPort]}
          error={[portErr, setPortErr]}
        />
      </div>
      {parseInt(port) !== settings.globalSettings.data.port && (
        <div className="options">
          <Button onClick={updatePort}>Update</Button>
          <Button type="secondary" onClick={undoUpdatePort}>
            Cancel
          </Button>
        </div>
      )}
    </section>
  );
}

export default General;
