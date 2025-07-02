import { useCallback } from "react";
import { useSelector } from "react-redux";
import { useParams } from "react-router";

import ScanIcon from "assets/Icons/Scan";

function Rescan() {
  const params = useParams();
  const auth = useSelector((store) => store.auth);

  const handleClick = useCallback(async () => {
    const config = {
      method: "POST",
      headers: {
        "content-type": "application/json",
        authorization: auth.token,
      },
    };

    await fetch(`/api/v1/library/${params.id}/scan`, config);
  }, [auth.token, params.id]);

  return (
    <button className="rescan" onClick={handleClick}>
      Rescan library
      <ScanIcon />
    </button>
  );
}

export default Rescan;
