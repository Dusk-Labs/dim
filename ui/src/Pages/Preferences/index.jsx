import { Component } from "react";
import { connect } from "react-redux";
import { fetchInvites, createNewInvite } from "../../actions/auth.js";

import ProfileTab from './tabs/Profile';
import AppearanceTab from './tabs/Appearance';
import AdvancedTab from "./tabs/Advanced";

import "./Preferences.scss";

class Preferences extends Component {
    constructor(props) {
        super(props);
        this.switchTo = this.switchTo.bind(this);
        this.state = {
            active: 0,
            tabs: []
        }
    }

    componentDidMount() {
        document.title = "Dim - Preferences";

        if (this.props.user.info.owner) {
            this.props.fetchInvites();
        }

    }

    componentDidUpdate() {}

    switchTo(index) {
        this.setState({
            active: index
        });
    }

    render() {
        this.state.tabs = [<ProfileTab user={this.props.user}/>, <AppearanceTab/>, <AdvancedTab/>]
        return (
            <div className="preferencesPage">
                <div className="preferences">
                    <nav>
                        <section className="tabContentSection">
                            <p>Preferences</p>
                            <div className="fields">
                                <div className={this.state.active === 0 ? "field active" : "field"} onClick={() => this.switchTo(0)}>
                                    <p>Account</p>
                                </div>
                                <div className={this.state.active === 1 ? "field active" : "field"} onClick={() => this.switchTo(1)}>
                                    <p>Appearance</p>
                                </div>
                                <div className={this.state.active === 2 ? "field active" : "field"} onClick={() => this.switchTo(2)}>
                                    <p>Advanced</p>
                                </div>
                            </div>
                        </section>
                    </nav>
                    <div className="content">
                        <section>
                            {this.props.user.fetched ? 
                            this.state.tabs[this.state.active] : null}
                        </section>
                    </div>
                </div>
            </div>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.auth,
    user: state.user
});

const mapActionsToProps = {
    fetchInvites,
    createNewInvite
};

export default connect(mapStateToProps, mapActionsToProps)(Preferences);
