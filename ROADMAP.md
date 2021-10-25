# Mission
Our mission is to create a ecosystem of open source applications with high UX value backed by a backend that scales effortlessly. Key parts of our mission include improving the video player experience, on which we will focus as much as possible, and native clients.

# Roadmap
## Media management experience

The following list contains various milestones containing features that would make the lives of administrators a lot easier. Amongst these, there are features which improve the UX of rematching a media that has been matched incorrectly, and features that allow admins to cherry pick metadata of media or bring in their own. 

Lastly we have features which will allow administrators to globally change default user settings, and features that will show users information about their server such as currently active streams, a live view into the streaming scheduler, and various other statistics.

| Status | Goal | Labels | Repository | Target |
| :---: | :--- | --- | --- | --- |
| ❌ | [Better rematching](https://github.com/Dusk-Labs/dim/issues/36) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q4 2021 |
| ❌ | [Manual metadata entry](https://github.com/Dusk-Labs/dim/issues/274) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q4 2021 |
| ❌ | [Better admin settings](https://github.com/Dusk-Labs/dim/issues/254) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q1 2022 |
| ❌ | [Add activity page](https://github.com/Dusk-Labs/dim/issues/45) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q2 2022 |


## Player experience

Progress of these milestones are tracked in <a href=https://github.com/Dusk-Labs/dim/issues/292>Issue 292</a>.

The following list contains milestones representing features that improve the users experience when playing media in their clients. Our main priority right now is improving how dim handles various video formats. We want to reduce cases where users will need to transcode their media, and as such we will be implementing native subtitle rendering for subtitles like `VOBSUB` and `PGS`.

We also want to improve the debugging experience in case we encounter a format that we dont handle well, as such we also want to improve the error reporting of the player, as to make it easier to submit bug reports and to make bug reports easier to investigate.

| Status | Goal | Labels | Repository | Target |
| :---: | :--- | --- | --- | --- |
| ❌ | [Native Subtitle rendering](https://github.com/Dusk-Labs/dim/issues/162) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q4 2021 |
| ❌ | [Improved player error reporting](https://github.com/Dusk-Labs/dim/issues/109) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q4 2021 |
| ❌ | [Improved handling of HDR media](https://github.com/Dusk-Labs/dim/issues/246) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q1 2022 |


## Native Clients experience

The following list contains milestones related to the development of native clients. At the moment the most important milestones are improving API documentations and defining initial application features which directly block the development of the native clients.

| Status | Goal | Labels | Repository | Target |
| :---: | :--- | --- | --- | --- |
| ❌ | [Improve API documentation and error codes](https://github.com/Dusk-Labs/dim/issues/276) | | <a href=https://github.com/Dusk-Labs/dim>Dusk-Labs/dim</a> | Q4 2021 |
| ❌ | [Define initial application features](https://github.com/Dusk-Labs/dim-mobile/issues/2) | | <a href=https://github.com/Dusk-Labs/dim-mobile>Dusk-Labs/dim-mobile</a> | Q1 2022 |
| ❌ | [Create initial application designs](https://github.com/Dusk-Labs/dim-mobile/issues/1) | | <a href=https://github.com/Dusk-Labs/dim-mobile>Dusk-Labs/dim-mobile</a> | Q2 2022 |
| ❌ | Develop mobile client | | <a href=https://github.com/Dusk-Labs/dim-mobile>Dusk-Labs/dim-mobile</a> | Q2/Q3 2022 |
| ❌ | Develop Android TV client | | <a href=https://github.com/Dusk-Labs/dim-mobile>Dusk-Labs/dim-mobile</a> | - |
| ❌ | Develop Apple TV client | | Dusk-Labs/dim-atv | - |
