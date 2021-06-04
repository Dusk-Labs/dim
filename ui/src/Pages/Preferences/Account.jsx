import UserCard from "../../Components/CardList/UserCard";

function AccountSection(props) {
  return (
    <div className="accountSectionParent">
      <span className="accountHeadingText">{props.heading}</span>
      <div className="accountSectionContent">
        {props.children}
      </div>
    </div>
  );
}

function Account() {
  return (
    <>
      <section className="accountSection">
        <AccountSection heading="My Account">
          <label>Username</label>
          <input className="inputField" placeholder="Enter username..."/>
          <label className="multiSelectLabel">Language</label>
          <div className="inputSelect">

          </div>
        </AccountSection>
        <AccountSection heading="Password and Authentication">
          <label>Password</label>
          <input type="password" className="inputField" placeholder="Enter password..."/>
        </AccountSection>
        <AccountSection heading="Account removal">
          <label>Delete account</label>
          <button className="removeAccountButton">Delete</button>
        </AccountSection>
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
                <th className="removeHeader">Remove</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>9ebe4e49d6244e12ba7c2e1e5a15aa59</td>
                <td>2</td>
                <td>Liam</td>
                <td>1</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </>
  );
}

export default Account;
