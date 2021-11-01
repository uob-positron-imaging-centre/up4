use ndarray::prelude::*;
use super::buffer::Buffer;

// return the number of timesteps
pub fn timesteps(file: &hdf5::File) -> u64 {
    let timesteps: u64 = file.attr("timesteps").expect(
        &format!("Can not find attribute \"timesteps\" in file {}", file.filename())
    )
    .read_scalar()
    .expect(&format!("Can not read attribute \"timesteps\" in file {}", file.filename()));
    // return
    timesteps
}


pub fn position<'r>(file: &hdf5::File,buffer: &'r Buffer, timestep: u64)->ArrayView2<'r,f64>{
        let position = buffer.get::<Ix2>("position",timestep);
        position
}

pub fn velocity<'r>(file: &hdf5::File,buffer: &'r Buffer, timestep: u64)->ArrayView2<'r,f64>{
    let velocity = buffer.get::<Ix2>("velocity",timestep);
    velocity

}

pub fn radius<'r>(file: &hdf5::File,buffer: &'r Buffer, timestep: u64)->ArrayView<'r,f64,Ix1>{
        let radius = buffer.get::<Ix1>("radius",timestep);
        radius
}

pub fn particleid<'r>(file: &hdf5::File,buffer: &'r Buffer, timestep: u64)->ArrayView1<'r,f64>{
        let parti_id = buffer.get::<Ix1>("particleid",timestep);
        parti_id
}

pub fn time(file: &hdf5::File, buffer: &Buffer, timestep: u64)->f64{
        // This function assuming that eaach particle has the same timesteps!!
        file.group(&format!("particle {}",0))
            .expect(&format!("Can not fine particle {} in file {}",0, file.filename()))
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                file.filename()
            ))
            .read_1d::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                file.filename()
            ))[timestep as usize]
}

pub fn clouds<'r>(file: &hdf5::File,buffer: &'r Buffer, timestep: u64)->ArrayView1<'r,f64>{
    let clouds = buffer.get::<Ix1>("ppcloud",timestep);
    clouds
}


pub fn get_dt(file: &hdf5::File,buffer: &Buffer, timestep: u64) -> f64 {
    file.attr("sample rate")
    .expect(&format!(
        "Can not find attribute \"sample rate\" in HDF5 file \"{:?}\"",
        file.filename()
    ))
    .read_scalar()
    .expect(&format!(
        "Can not read attribute \"sample rate\" in HDF5 file \"{:?}\"",
        file.filename()
    ))
}

pub fn update(file: &hdf5::File,buffer: &mut Buffer, timestep: u64){
    if !buffer.has(timestep){
        update_buffer_all(file, buffer)
    }
}

fn update_buffer_all(file: &hdf5::File,buffer: &mut Buffer){
    let timesteps = timesteps(file) as isize;
    let mut buffersize = buffer.buffersize;
    let mut range = buffer.range() + buffersize as isize ;
    if range[1] > timesteps{
        range[1] = timesteps-1;
        buffersize = (range[1] -range[0]) as usize+1;
        buffer.buffersize=buffersize;
    }
    let particles: usize = file.attr("particle number").expect(
        &format!("Can not find attribute \"particle number\" in file {}", file.filename())
    )
    .read_scalar()
    .expect(&format!("Can not read attribute \"particle number\" in file {}", file.filename()));
    let mut position: Array3<f64> = Array3::<f64>::zeros((buffersize,particles,3));
    let mut velocity: Array3<f64> = Array3::<f64>::zeros((buffersize,particles,3));
    let mut radius: Array2<f64> = Array2::<f64>::zeros((buffersize,particles));
    let mut particleid: Array2<f64> = Array2::<f64>::zeros((buffersize,particles));
    let mut clouds: Array2<f64> = Array2::<f64>::ones((buffersize,particles));
    for particle_id in 0..particles{
        // for each particle find the Position at the
        let part_pos = file.group(&format!("particle {}",particle_id))
                        .expect(&format!("Can not find group \"particle {}\" in file {}",
                            particle_id,
                            file.filename()))
                        .dataset("position")
                        .expect(&format!("Can not find dataset \"position\" in file {}",
                            file.filename()))
                        .read_2d::<f64>()
                        .expect(&format!("Can not read dataset \"position\" in file {}",
                            file.filename())).slice(s![range[0]..range[1]+1,..]).to_owned();

        position.slice_mut(s![..,particle_id,..])
                .assign(&part_pos);

        // for each particle find the velocuty at the
        let part_vel = file.group(&format!("particle {}",particle_id))
                        .expect(&format!("Can not find group \"particle {}\" in file {}",
                            particle_id,
                            file.filename()))
                        .dataset("velocity")
                        .expect(&format!("Can not find dataset \"velocity\" in file {}",
                            file.filename()))
                        .read_2d::<f64>()
                        .expect(&format!("Can not read dataset \"velocity\" in file {}",
                            file.filename())).slice(s![range[0]..range[1]+1,..]).to_owned();
        velocity.slice_mut(s![..,particle_id,..])
                .assign(&part_vel);
        //Radius
        let part_rad = file.group(&format!("particle {}",particle_id))
            .expect(&format!("Can not fine particle {} in file {}",particle_id, file.filename()))
            .attr("radius")
            .expect(&format!(
                "Can not find attribute \"radius\" in HDF5 file \"{:?}\"",
                file.filename()
            ))
            .read_scalar::<f64>()
            .expect(&format!(
                "Can not read data from \"radius\" attribute. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                file.filename()
            ));
        radius.slice_mut(s![0..1,particle_id])
                .assign(&array![ part_rad]);

        // TODO this function is actually not accessing data.
        // this should be fine if the conversion happens in order and
        // multiple particles are not yet implemented
        let particles: usize = file.attr("particle number").expect(
            &format!("Can not find attribute \"particle number\" in file {}", file.filename())
        )
        .read_scalar()
        .expect(&format!("Can not read attribute \"particle number\" in file {}", file.filename()));
        let mut part_id: Array1<f64> = Array1::<f64>::zeros(particles);
        for particle_id in 0..particles{
            part_id[particle_id] = particle_id as f64;
        }
        particleid.slice_mut(s![..,particle_id])
                .assign( &part_id);
    }
    buffer.update("position",position.into_dyn(),range.clone());
    buffer.update("velocity",velocity.into_dyn(),range.clone());
    buffer.update("radius",radius.into_dyn(),range.clone());
    buffer.update("particleid",particleid.into_dyn(),range.clone());
    buffer.update("ppcloud",clouds.into_dyn(),range.clone());
}
