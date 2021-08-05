import { useSelector } from "react-redux";

import Toast from "./Toast";

import "./Notifications.scss";

function Notifications() {
  const notifs = useSelector(state => state.notifications);

  return (
    <div className="notifications">
      {notifs.list.map((notif, i) => (
        <Toast key={i} id={i}>
          <p>{notif.msg}</p>
        </Toast>
      ))}
    </div>
  );
}

export default Notifications;
