import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchInvites, createNewInvite } from "../../actions/auth.js";

function Invites() {
  const dispatch = useDispatch();

  const { auth } = useSelector(store => ({
    auth: store.auth
  }));

  useEffect(() => {
    if (!auth.admin_exists) return;
    dispatch(fetchInvites());
  }, [auth.admin_exists, dispatch]);

  let invitesList = <p>You do not have permission to manage invites.</p>;

  if (auth.admin_exists) {
    const { invites, createNewInvite } = auth;

    const invs = [
      ...invites.items,
      createNewInvite.code
    ];

    // FETCH_INVITES_OK
    // if (!fetchInvites.fetching && fetchInvites.fetched && !fetchInvites.error) {
    if (true) {
      const invitesElement = invs.map((invite, i) => <p key={i}>{invite}</p>);

      invitesList = (
        invs.length === 0
          ? <p>You don't have any invite codes.</p>
          : <div className="invitesList">{invitesElement}</div>
      );
    }
  }

  return (
    <section className="invites">
      <p>Invite Codes</p>
      {auth.admin_exists &&
        <button onClick={() => dispatch(createNewInvite())}>Create new invite code</button>
      }
      {invitesList}
    </section>
  );
}

export default Invites;
