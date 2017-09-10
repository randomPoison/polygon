use polygon::geometry::mesh::*;
use std::path::Path;
use tobj;

pub fn load_mesh<P: AsRef<Path>>(path: P) -> Result<Mesh, BuildMeshError> {
    let (meshes, _) = tobj::load_obj(path.as_ref()).unwrap();
    let mesh = &meshes[0].mesh;
    let positions = mesh.positions.chunks(3).map(Into::into).collect::<Vec<_>>();
    let normals = mesh.normals.chunks(3).map(Into::into).collect::<Vec<_>>();
    let texcoords = mesh.texcoords.chunks(2).map(Into::into).collect::<Vec<_>>();

    MeshBuilder::new()
        .set_position_data(&*positions)
        .set_normal_data(&*normals)
        .set_texcoord_data(&*texcoords)
        .set_indices(&*mesh.indices)
        .build()
}
