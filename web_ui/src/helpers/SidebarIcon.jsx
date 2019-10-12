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
            default:
                icon = "folder";
                break;
        }

        return <FontAwesomeIcon icon={icon}/>
    }
}

export default SidebarIcon;
