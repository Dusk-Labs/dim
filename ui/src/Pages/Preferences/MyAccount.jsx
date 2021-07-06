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

function MyAccount() {
  return (
    <>
      <section className="accountSection">
        <AccountSection heading="My Account">
          <label>Username</label>
          <input className="inputField" placeholder="Enter username..."/>
        </AccountSection>
        <AccountSection heading="Password and Authentication">
          <label>Password</label>
          <input type="password" className="inputField" placeholder="Enter password..."/>
          <label>Confirm password</label>
          <input type="password" className="inputField" placeholder="Confirm password..."/>
        </AccountSection>
        <AccountSection heading="Account removal">
          <label>Delete account</label>
          <button className="removeAccountButton">Delete</button>
        </AccountSection>
      </section>
    </>
  );
}

export default MyAccount;
