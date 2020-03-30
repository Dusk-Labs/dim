import React, { Component } from "react";
import { connect } from "react-redux";

import "./Preferences.scss";

class Preferences extends Component {
    constructor(props) {
        super(props);

        this.switchTo = this.switchTo.bind(this);

        this.state = {
            active: 0
        }
    }

    componentDidMount() {
        document.title = `Dim - Preferences`;
    }

    componentDidUpdate() {}

    switchTo(index) {
        this.setState({
            active: index
        });
    }

    // TODO: improve nav switch - just added somethin' super basic to make it work.
    render() {
        const fields = [
            "Account",
            "Invites"
        ];

        return (
            <div className="preferencesPage">
                <div className="preferences">
                    <nav>
                        <section>
                            <p>Preferences</p>
                            <div className="fields">
                                {fields.map((field, i) => {
                                    const classes = this.state.active === i ? "field active" : "field";

                                    return (
                                        <div key={i} className={classes} onClick={() => this.switchTo(i)}>
                                            <p>{field}</p>
                                        </div>
                                    );
                                })}
                            </div>
                        </section>
                    </nav>
                    <div className="content">
                        {this.state.active === 0 &&
                            <section>
                                <p>My account</p>
                            </section>
                        }
                        {this.state.active === 1 &&
                            <section>
                                <p>Invites</p>
                            </section>
                        }
                    </div>
                </div>
            </div>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.auth
});

const mapActionsToProps = {};

export default connect(mapStateToProps, mapActionsToProps)(Preferences);
