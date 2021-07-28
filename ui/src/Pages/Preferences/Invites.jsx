import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { formatHHMMSSDate } from "../../Helpers/utils";
import { fetchInvites, createNewInvite, delInvite } from "../../actions/auth.js";
import TrashIcon from "../../assets/Icons/Trash";

import "./Invites.scss";

function Invites() {
  const dispatch = useDispatch();

  const { user, auth } = useSelector(store => ({
    user: store.user,
    auth: store.auth
  }));

  useEffect(() => {
    dispatch(fetchInvites());
  }, [auth.admin_exists, dispatch]);

  const genNewToken = useCallback(async () => {
    await dispatch(createNewInvite());
    dispatch(fetchInvites());
  }, [dispatch]);

  const delInviteToken = useCallback(async (token) => {
    await dispatch(delInvite(token));
    dispatch(fetchInvites());
  }, [dispatch]);

  const tokens = auth.invites.items.map((token, i) => {
    const {hours, mins, secs, date, month, year} = formatHHMMSSDate(token.created);

    return (
      <div className="token" key={i}>
        <p>{token.id}</p>
        <p>{hours}:{mins}:{secs} on the {date}/{month}/{year}</p>
        {token.claimed_by
          ? <p>{token.claimed_by}</p>
          : <p>Available</p>
        }
        {user.info.username !== token.claimed_by && (
          <button onClick={() => delInviteToken(token.id)}>
            <TrashIcon/>
          </button>
        )}
      </div>
    );
  });

  return (
    <div className="preferencesInvites">
      <section>
        <h2>Manage invites</h2>
        <p className="desc">Create a token to invite someone and allow them access to your added media libraries securely.</p>
        <h3>Tokens</h3>
        <div className="tokensContainer">
          <div className="heading">
            <p>ID</p>
            <p>Created at</p>
            <p>Status</p>
          </div>
          <div className="separator"/>
          <div className="tokens">
            {tokens}
          </div>
        </div>
        <button className="genTokenBtn" onClick={genNewToken}>
          Generate a new token
        </button>
      </section>
    </div>
  );
}

export default Invites;
