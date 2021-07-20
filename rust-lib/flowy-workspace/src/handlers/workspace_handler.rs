use crate::{
    entities::{app::RepeatedApp, workspace::*},
    errors::WorkspaceError,
    services::WorkspaceController,
};
use flowy_dispatch::prelude::{response_ok, Data, ModuleData, ResponseResult};
use std::{convert::TryInto, sync::Arc};

pub async fn create_workspace(
    data: Data<CreateWorkspaceRequest>,
    controller: ModuleData<Arc<WorkspaceController>>,
) -> ResponseResult<Workspace, WorkspaceError> {
    let controller = controller.get_ref().clone();
    let params: CreateWorkspaceParams = data.into_inner().try_into()?;
    let detail = controller.save_workspace(params).await?;
    response_ok(detail)
}

pub async fn get_cur_workspace(
    controller: ModuleData<Arc<WorkspaceController>>,
) -> ResponseResult<Workspace, WorkspaceError> {
    let workspace = controller.get_cur_workspace().await?;
    response_ok(workspace)
}

pub async fn get_workspace(
    data: Data<QueryWorkspaceRequest>,
    controller: ModuleData<Arc<WorkspaceController>>,
) -> ResponseResult<Workspace, WorkspaceError> {
    let params: QueryWorkspaceParams = data.into_inner().try_into()?;
    let mut workspace = controller.get_workspace(&params.workspace_id).await?;

    if params.read_apps {
        let apps = controller.get_apps(&params.workspace_id).await?;
        workspace.apps = RepeatedApp { items: apps };
    }

    response_ok(workspace)
}
