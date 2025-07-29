import { useAppSelector } from "../hooks/store";

import Toast from "./Toast";

import "./Notifications.scss";

function Notifications() {
  const notifs = useAppSelector((state) => state.notifications);

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
