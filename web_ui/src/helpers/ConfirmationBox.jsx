import React, { Component, Fragment } from "react";
import Modal from "react-modal";
import { connect } from "react-redux";

import "./ConfirmationBox.scss";

class ConfirmationBox extends Component {
    constructor(props) {
        super(props);

        this.state = {
            visible: false
        };
    }

    open = () => this.setState({visible: true});
    close = () => this.setState({visible: false});

    confirm = () => {
        this.props.continue();
        this.close();
    }

    render() {
        return (
            <Fragment>
                <button onClick={this.open}>-</button>
                <Modal
                    isOpen={this.state.visible}
                    contentLabel="newLibrary"
                    className="confirmationBox"
                    onRequestClose={this.close}
                    overlayClassName="popupOverlay"
                >
                    <h3>CONFIRM ACTION</h3>
                    <p>{this.props.message}</p>
                    <div className="options">
                        <button className="confirmationBoxCancel" onClick={this.close}>CANCEL</button>
                        <button className="confirmationBoxContinue" onClick={this.confirm}>{this.props.action}</button>
                    </div>
                </Modal>
            </Fragment>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer
});

const mapActionsToProps = {};

export default connect(mapStateToProps, mapActionsToProps)(ConfirmationBox);
