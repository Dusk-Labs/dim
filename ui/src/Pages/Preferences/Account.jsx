import UserCard from "../../Components/CardList/UserCard";

function Account() {
  return (
    <>
      <section className="accountSection">
        <div/>
        <div/>
        <div/>
      </section>
      <section className="usersSection">
        <div className="sectionHeading">
          <span>Users</span>
          <button className="editBtn">
            edit
          </button>
        </div>
        <div className="userListContainer">
          <UserCard user={{"name": "placeholder", "image": "https://i.redd.it/3n1if40vxxv31.png"}}/>
          <UserCard user={{"name": "placeholder2", "image": "https://i.redd.it/3n1if40vxxv31.png"}}/>
        </div>
      </section>
      <section className="tokenSection">
        <div className="sectionHeading">
          <span>Tokens</span>
          <button className="editBtn">
            add
          </button>
        </div>
        <div className="tableSection">
          <table className="tokenTable">
            <tr>
              <th style={{width: "65%"}}>Token</th>
              <th>Created</th>
              <th>Claimed by</th>
              <th>Remove</th>
            </tr>
            <tr>
              <td>aksdflkjaslkdjflkajsdlkfjalksjdfl;kjasldkjf</td>
              <td>2</td>
            </tr>
          </table>
        </div>
      </section>
    </>
  );
}

export default Account;
