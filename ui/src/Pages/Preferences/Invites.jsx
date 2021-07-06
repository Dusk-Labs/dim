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
    const { invites } = auth;

    const invs = invites.items;

    if (!invites.fetching && invites.fetched && !invites.error) {
      console.log(invs);
      const invitesElement = invs.map((invite, i) => {
        return (
          <tr>
            <td>{invite.id}</td>
            <td>{invite.created}</td>
            <td>{invite.claimed_by}</td>
          </tr>
        );
      });

      invitesList = (
        invs.length === 0
          ? <p>You don't have any invite codes.</p>
          : <tbody>{invitesElement}</tbody>
      );
    }
  }

  return (
    <section className="tokenSection">
      <div className="sectionHeading">
        <span>Tokens</span>
        <button className="editBtn" onClick={() => dispatch(createNewInvite())}>
          new
        </button>
      </div>
      <div className="tableSection">
        <table className="tokenTable">
          <thead>
            <tr>
              <th style={{width: "65%"}}>Token</th>
              <th>Created</th>
              <th>Claimed by</th>
            </tr>
          </thead>
          {invitesList}
        </table>
      </div>
    </section>
  );
}

export default Invites;
