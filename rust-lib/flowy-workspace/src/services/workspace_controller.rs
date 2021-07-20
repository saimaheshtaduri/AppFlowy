use crate::{
    entities::{app::App, workspace::*},
    errors::*,
    module::{WorkspaceDatabase, WorkspaceUser},
    services::AppController,
    sql_tables::workspace::{WorkspaceSql, WorkspaceTable, WorkspaceTableChangeset},
};
use flowy_dispatch::prelude::DispatchFuture;
use std::sync::Arc;

pub struct WorkspaceController {
    pub user: Arc<dyn WorkspaceUser>,
    pub sql: Arc<WorkspaceSql>,
    pub app_controller: Arc<AppController>,
}

impl WorkspaceController {
    pub fn new(
        user: Arc<dyn WorkspaceUser>,
        database: Arc<dyn WorkspaceDatabase>,
        app_controller: Arc<AppController>,
    ) -> Self {
        let sql = Arc::new(WorkspaceSql { database });
        Self {
            user,
            sql,
            app_controller,
        }
    }

    pub async fn save_workspace(
        &self,
        params: CreateWorkspaceParams,
    ) -> Result<Workspace, WorkspaceError> {
        let workspace_table = WorkspaceTable::new(params);
        let detail: Workspace = workspace_table.clone().into();
        let _ = self.sql.write_workspace_table(workspace_table)?;
        let _ = self.user.set_cur_workspace_id(&detail.id).await?;

        Ok(detail)
    }

    pub fn update_workspace(&self, params: UpdateWorkspaceParams) -> Result<(), WorkspaceError> {
        let changeset = WorkspaceTableChangeset::new(params);
        let _ = self.sql.update_workspace(changeset)?;

        Ok(())
    }

    pub fn delete_workspace(&self, workspace_id: &str) -> Result<(), WorkspaceError> {
        unimplemented!()
    }

    pub async fn get_cur_workspace(&self) -> Result<Workspace, WorkspaceError> {
        let user_workspace = self.user.get_cur_workspace().await?;
        let workspace = self.get_workspace(&user_workspace.workspace_id).await?;
        Ok(workspace)
    }

    pub async fn get_cur_apps(&self) -> Result<Vec<App>, WorkspaceError> {
        let user_workspace = self.user.get_cur_workspace().await?;
        let apps = self.get_apps(&user_workspace.workspace_id).await?;
        Ok(apps)
    }

    pub async fn get_workspace(&self, workspace_id: &str) -> Result<Workspace, WorkspaceError> {
        let workspace_table = self.get_workspace_table(workspace_id).await?;
        Ok(workspace_table.into())
    }

    pub async fn get_apps(&self, workspace_id: &str) -> Result<Vec<App>, WorkspaceError> {
        let apps = self
            .sql
            .read_apps_belong_to_workspace(workspace_id)?
            .into_iter()
            .map(|app_table| app_table.into())
            .collect::<Vec<App>>();

        Ok(apps)
    }

    fn get_workspace_table(
        &self,
        workspace_id: &str,
    ) -> DispatchFuture<Result<WorkspaceTable, WorkspaceError>> {
        let sql = self.sql.clone();
        let workspace_id = workspace_id.to_owned();
        DispatchFuture {
            fut: Box::pin(async move {
                let workspace = sql.read_workspace(&workspace_id)?;
                // TODO: fetch workspace from remote server
                Ok(workspace)
            }),
        }
    }
}
