import { useState, useCallback, useEffect } from "react";

import Toggle from "Components/Toggle";

function EnableSsa() {
  const [enableSsa, setEnableSsa] = useState(
    localStorage.getItem("enable_ssa") === "true"
  );

  useEffect(() => {
    localStorage.setItem("enable_ssa", enableSsa.toString());
  }, [enableSsa]);

  const handleToggle = useCallback(
    (state) => {
      setEnableSsa(state);
    },
    [setEnableSsa]
  );

  return (
    <section>
      <h2>Subtitle settings</h2>
      <Toggle
        onToggle={handleToggle}
        state={enableSsa}
        name="Enable experimental support for SSA/ASS subtitles."
      />
    </section>
  );
}

export default EnableSsa;
