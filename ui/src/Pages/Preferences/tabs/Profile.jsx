const FormRow = ({ options }) => {
    return (
        <div className="formRowContainer">
            <div className="formRowLabel">
                <label className="formLabelText">{options.labelText}</label>
                <p className="formValueText">{options.value}</p>
            </div>
            <div>
                <button className="editBtn">Edit</button>
            </div>
        </div>
    )
}

export default function ProfileTab (props) {
    return (
        <div className="tabContainer">
            <div className="tabPreference">
                <div className="profileContainer">
                    <div className="profilePreview">
                        <div className="pfpContainer">
                            <img src={props.user.info.picture} className="pfpImage"/>
                        </div>
                        <p>{props.user.info.username}</p>
                    </div>
                    <div className="tabSubContainer">
                        <FormRow options={{
                            labelText: 'Username',
                            value: props.user.info.username
                        }}/>
                        <FormRow options={{
                            labelText: 'Password',
                            value: "***********"
                        }}/>
                        <FormRow options={{
                            labelText: 'Language',
                            value: "English"
                        }}/>
                    </div>
                </div>
                <div className="tabSubContainer">
                    <div>Invitation tokens</div>
                </div>
            </div>
        </div>
    )
}