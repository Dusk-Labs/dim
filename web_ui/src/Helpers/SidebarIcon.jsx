import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

class SidebarIcon extends Component {
    render() {
        let icon;

        switch(this.props.icon.toLowerCase()) {
            case "movies":
                icon = "film";
                break;
            case "movie":
                icon = "film";
                break;
            case "tv":
                icon = "tv";
                break;
            case "desktop":
                icon = "desktop";
                break;
            case "laptop":
                icon = "laptop";
                break;
            case "phone":
                icon = "mobile-alt";
                break;
            case "dashboard":
                icon = "home"
                break;
            case "preferences":
                icon = "wrench";
                break;
            case "logout":
                icon = "door-open"
            default:
                icon = "folder";
                break;
        }

        return <FontAwesomeIcon className="item-wrapper-icon" icon={icon}/>
    }
}

export default SidebarIcon;
