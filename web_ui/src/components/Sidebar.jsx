import React, { Component } from 'react';

class Sidebar extends Component {
    render() {
        return (
            <div className="dim-sidebar-wrapper">
                <sidebar className="dim-sidebar">
                    <div className="dim-sidebar-section dim-sidebar-profile">
                        <div className="profile-icon">
                            <div className="profile-icon-inner">
                            </div>
                        </div>
                        <h4 className="profile-name">Username</h4>
                        <h6 className="profile-hours">0h spent watching</h6>
                        <div className="separator"></div>
                    </div>

                    <div className="dim-sidebar-section dim-sidebar-scrollable">
                        <h4 className="label"> CONNECTED HOSTS </h4>
                        <ul className="dim-list dim-hosts">
                            <li className="item">
                                <i class="fas fa-desktop"></i>
                                Desktop
                            </li>
                            <li className="item">
                                <i class="fas fa-laptop"></i>
                                Laptop
                            </li>
                            <li className="item">
                                <i class="fas fa-mobile-alt"></i>
                                Phone
                            </li>
                        </ul>
                    </div>

                    <div className="dim-sidebar-section dim-sidebar-scrollable">
                        <h4 className="label"> LOCAL LIBRARIES </h4>
                        <ul className="dim-list dim-libraries">
                            <li className="item">
                                <i class="fas fa-film"></i>
                                Movies
                            </li>
                            <li className="item">
                                <i class="fas fa-gamepad"></i>
                                Games
                            </li>
                            <li className="item">
                                <i class="fas fa-tv"></i>
                                TV Shows
                            </li>
                        </ul>
                    </div>

                    <div className="dim-sidebar-section dim-sidebar-account-opt">
                        <h4> Preferences </h4>
                        <h4> Legal </h4>
                        <h4> Logout </h4>
                    </div>
                </sidebar>
            </div>
            );
    }
}

export default Sidebar;
