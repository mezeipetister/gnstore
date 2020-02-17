// Copyright (C) 2020 Peter Mezei
//
// This file is part of GNStore.
//
// GNStore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// GNStore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNStore.  If not, see <http://www.gnu.org/licenses/>.

use crate::guard::Login;
use crate::prelude::*;
use crate::DataLoad;
use chrono::prelude::*;
use core_lib::error::Error;
use core_lib::model::*;
use core_lib::prelude::AppResult;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use storaget::StorageObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewIssue {
    /**
     * Issue title
     */
    title: String,
    /**
     * Issue description
     */
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueShort {
    id: String,
    title: String,
    description: String,
    created_by: String,
    pub date_created: DateTime<Utc>,
    labels: Vec<Label>,
    assigned_to: String,
    comment_count: usize,
    is_open: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueLong {
    id: String,
    title: String,
    description: String,
    created_by: String,
    date_created: DateTime<Utc>,
    labels: Vec<Label>,
    assigned_to: String,
    comment_count: usize,
    events: Vec<Event>,
    followed_by: Vec<String>,
    is_open: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentNew {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelScheme {
    subject: String,
    text_color: String,
    background_color: String,
}

impl From<Issue> for IssueShort {
    fn from(issue: Issue) -> Self {
        IssueShort {
            id: issue.get_id().to_string(),
            title: issue.get_title(),
            description: issue.get_description(),
            created_by: issue.get_created_by(),
            date_created: issue.get_date_created(),
            labels: issue.get_labels(),
            assigned_to: issue.get_assigned_to(),
            comment_count: issue.get_comment_count(),
            is_open: issue.get_is_open(),
        }
    }
}

impl From<Issue> for IssueLong {
    fn from(issue: Issue) -> Self {
        IssueLong {
            id: issue.get_id().to_string(),
            title: issue.get_title(),
            description: issue.get_description(),
            created_by: issue.get_created_by(),
            date_created: issue.get_date_created(),
            labels: issue.get_labels(),
            assigned_to: issue.get_assigned_to(),
            comment_count: issue.get_comment_count(),
            events: issue.get_events(),
            followed_by: issue.get_followed_by(),
            is_open: issue.get_is_open(),
        }
    }
}

impl From<Label> for LabelScheme {
    fn from(label: Label) -> Self {
        LabelScheme {
            subject: label.get_subject(),
            text_color: label.get_text_color(),
            background_color: label.get_background_color(),
        }
    }
}

#[put("/issue/new", data = "<form>")]
pub fn user_new_put(
    user: Login,
    data: State<DataLoad>,
    form: Json<NewIssue>,
) -> Result<StatusOk<IssueShort>, ApiError> {
    let new_issue = Issue::new(
        form.title.clone(),
        form.description.clone(),
        user.userid().to_string(),
    );
    data.inner().issues.add_to_storage(new_issue.clone())?;
    Ok(StatusOk(new_issue.into()))
}

#[get("/issue/all")]
pub fn issue_all_get(
    _user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<IssueShort>>, ApiError> {
    let mut res = data
        .inner()
        .issues
        .into_iter()
        .map(|d| d.get(|i| i.clone().into()))
        .collect::<Vec<IssueShort>>();
    /*
     * Order result by date
     */
    res.sort_by(|a, b| b.date_created.cmp(&a.date_created));
    Ok(StatusOk(res))
}

#[get("/issue/<id>")]
pub fn issue_id_get(
    _user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    let issue = data.inner().issues.get_by_id(&id)?.clone_data();
    Ok(StatusOk(issue.clone().into()))
}

#[post("/issue/<id>/follow")]
pub fn issue_id_follow_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.follow(user.userid().to_string());
                i.clone()
            });
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/unfollow")]
pub fn issue_id_unfollow_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.unfollow(user.userid().to_string());
                i.clone()
            });
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/assign_to/<assigned_to>")]
pub fn issue_id_assign_to_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    assigned_to: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    // Validate, assigned_to userid exist
    if let Err(_) = data.inner().users.get_by_id(&assigned_to) {
        return Err(ApiError::BadRequest(
            "A megadott user ID nem létezik, így nem lehet hozzárendelni az issue-hoz.".to_owned(),
        ));
    }
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.set_assigned_to(assigned_to.clone(), user.userid().to_string());
                i.clone()
            });
            // Send notification to the assigned user
            let mut notification = Notification::new(format!(
                "Hozzárendeltek a következő issue-hoz: {}",
                mod_issue.get_title()
            ));
            notification.set_location(Location::Issue {
                id: id.clone(),
                section: None,
            });
            let _ = notify_user(&assigned_to, data.inner(), notification);
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

fn notify_user(user: &str, data: &DataLoad, notification: Notification) -> AppResult<()> {
    if let Err(_) = data.users.get_by_id(user) {
        return Err(Error::BadRequest(
            "a megadott user nem létezik, nem tudjuk értesíteni".to_owned(),
        ));
    }
    match data.notifications.get_by_id(user) {
        Ok(container) => {
            container.update(|c| c.add(notification.clone()));
            Ok(())
        }
        Err(_) => {
            data.notifications
                .add_to_storage(NotificationContainer::new(user.to_string()))
                .unwrap();
            Ok(())
        }
    }
}

#[post("/issue/<id>/close")]
pub fn issue_id_close_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.close_issue(user.userid().to_string());
                i.clone()
            });
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/open")]
pub fn issue_id_open_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.open_issue(user.userid().to_string());
                i.clone()
            });
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/comment", data = "<form>")]
pub fn issue_id_comment_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    form: Json<CommentNew>,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> Issue {
                i.add_comment(form.text.clone(), user.userid().to_string());
                i.clone()
            });
            Ok(StatusOk(mod_issue.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/comment/<comment_id>/like")]
pub fn issue_id_comment_like_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    comment_id: usize,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> AppResult<Issue> {
                i.like_comment(comment_id, user.userid().to_string())?;
                Ok(i.clone())
            });
            Ok(StatusOk(mod_issue?.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/comment/<comment_id>/dislike")]
pub fn issue_id_comment_dislike_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    comment_id: usize,
) -> Result<StatusOk<IssueLong>, ApiError> {
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> AppResult<Issue> {
                i.dislike_comment(comment_id, user.userid().to_string())?;
                Ok(i.clone())
            });
            Ok(StatusOk(mod_issue?.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/label/add", data = "<label>")]
pub fn issue_id_label_add_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    label: Json<LabelScheme>,
) -> Result<StatusOk<IssueLong>, ApiError> {
    let lab = Label::new(
        label.subject.clone(),
        label.text_color.clone(),
        label.background_color.clone(),
    );
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> AppResult<Issue> {
                i.add_label(lab.clone(), user.userid().to_string());
                Ok(i.clone())
            });
            Ok(StatusOk(mod_issue?.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

#[post("/issue/<id>/label/remove", data = "<label>")]
pub fn issue_id_label_remove_post(
    user: Login,
    data: State<DataLoad>,
    id: String,
    label: Json<LabelScheme>,
) -> Result<StatusOk<IssueLong>, ApiError> {
    let lab = Label::new(
        label.subject.clone(),
        label.text_color.clone(),
        label.background_color.clone(),
    );
    match data.inner().issues.get_by_id(&id) {
        Ok(issue) => {
            let mod_issue = issue.update(|i| -> AppResult<Issue> {
                i.remove_label(lab.clone(), user.userid().to_string());
                Ok(i.clone())
            });
            Ok(StatusOk(mod_issue?.into()))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

/*
 * (+) follow / unfollow
 * (+) label add / remove
 * (+) assigned_to
 * (+) coment
 * (+) comment like / dislike
 * ( ) comment notify @ sign detection
 *     and Notification integration
 * (+) close / open
 */
