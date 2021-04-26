import { Component } from "react";
import { connect } from "react-redux";
import { fetchInvites, createNewInvite } from "../../actions/auth.js";

import ProfileTab from './Tabs/Profile';
import AppearanceTab from './Tabs/Appearance';
import AdvancedTab from "./Tabs/Advanced";

import React, {useRef, useState, useEffect} from 'react'

import "./Preferences.scss";

function Preferences(props) {
    const editBadge = useRef(null);
    const leftProfilePic = useRef(null);
    const badge = useRef(null);

    const [active, setActive] = useState(0)
    const [tabs, setTabs] = useState([])
    const [badgePos, setBagePos] = useState({right: 0, top: 0})

    const tempStats = [
        {"name" : "Watched", val: "33h"},
        {"name" : "Users", val: "4"},
        {"name" : "Tokens", val: "3"}
    ]

    function computeCirclePos() {
        let containerHeight = leftProfilePic.current.clientHeight
        let containerRadius = leftProfilePic.current.clientWidth / 2
        let badgeWidth = editBadge.current.clientWidth
        let smallBadgeWidth = badge.current.clientWidth

        setBagePos({

                top: (containerHeight / 2) - badgeWidth / 2,
                left: (containerRadius) + badgeWidth - smallBadgeWidth,
                width: containerRadius + smallBadgeWidth / 2

        })
    }

    useEffect(() => {
        document.title = "Dim - Preferences"
        computeCirclePos()
        if (props.user.info.owner) {
            props.fetchInvites();
        }

        setTabs([<ProfileTab user={props.user}/>, <AppearanceTab/>, <AdvancedTab/>])

    }, [])

    return (
        <div className="preferencesPage">
        <div className="preferences">
            <div className="leftBar">
                <div className="leftBarImgContainer">
                    <div ref={leftProfilePic} class="leftBarImgParent">
                        <img className="leftBarProfileImg" src={props.user.info.picture}/>
                        <div className="circle" style={badgePos} ref={editBadge}><div ref={badge} className="leftBarImgEdit"/></div>
                    </div>
                </div>
                <div className="leftBarNames">
                    <div className="leftBarUsername">
                        {props.user.info.username}
                    </div>
                    <div className="leftBarRole">
                        {"Admin"}
                    </div>
                </div>
                <div className="leftBarStatistics">
                    {tempStats.map((stat, i) => (
                        <div key={i} className="leftBarStat">
                            <div className="leftBarStatValue">{stat.val}</div>
                            <div className="leftBarStatName">{stat.name}</div>
                        </div>
                    ))}
                </div>
                <hr className="leftBarSep"/>
                <div className="leftBarTabs">
                    <div className={active == 0 && "active"} onClick={() => setActive(0)}>Account</div>
                    <div className={active == 1 && "active"} onClick={() => setActive(1)}>Appearance</div>
                    <div className={active == 2 && "active"} onClick={() => setActive(2)}>Advanced</div>
                </div>
            </div>
            <div className="content">
                {/* <section>
                    {this.props.user.fetched ? 
                    this.state.tabs[this.state.active] : null}
                </section> */}
            </div>
        </div>
    </div>
    )
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
