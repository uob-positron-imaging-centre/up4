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


pub fn position<'r>(file: &hdf5::File,buffer: &'r mut Buffer, timestep: u64)->ArrayView2<'r,f64>{
        if !buffer.has(timestep){
            update_buffer_all(file,  buffer)
        }
        let position = buffer.get::<Ix2>("position",timestep);
        position
}

pub fn velocity<'r>(file: &hdf5::File,buffer: &'r mut Buffer, timestep: u64)->ArrayView2<'r,f64>{


    if !buffer.has(timestep){
        update_buffer_all(file,  buffer)
    }
    let velocity = buffer.get::<Ix2>("velocity",timestep);
    velocity

}

pub fn radius<'r>(file: &hdf5::File,buffer: &'r mut Buffer, timestep: u64)->ArrayView<'r,f64,Ix1>{
        if !buffer.has(timestep){
            update_buffer_all(file, buffer)
        }
        let radius = buffer.get::<Ix1>("radius",timestep);
        radius
}


pub fn particleid<'r>(file: &hdf5::File,buffer: &'r mut Buffer, timestep: u64)->ArrayView<'r,f64,Ix1>{
    if !buffer.has(timestep){
        update_buffer_all(file, buffer)
    }
    let id = buffer.get::<Ix1>("particleid",timestep);
    id

}

pub fn time(file: &hdf5::File,buffer: &mut Buffer,timestep: u64)->f64{
        file.group(&format!("timestep {}",timestep))
            .expect(&format!("Can not fine timestep {} in file {}",timestep, file.filename()))
            .dataset("time")
            .expect(&format!(
                "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
                file.filename()
            ))
            .read_raw::<f64>()
            .expect(&format!(
                "Can not read data from \"time\" dataset. \
                Data type or data format might be wrong. \
                Check creation of HDF5 file  \"{:?}\"",
                file.filename()
            ))[0]
}

pub fn clouds<'r>(file: &hdf5::File,buffer: &'r mut Buffer, timestep: u64)->ArrayView1<'r,f64>{
    if !buffer.has(timestep){
        update_buffer_all(file, buffer)
    }
    let clouds = buffer.get::<Ix1>("ppcloud",timestep);
    clouds
}


pub fn get_dt(file: &hdf5::File,timestep: u64) -> f64 {
    let timesteps = timesteps(&file);
    let id1;
    let id2;
    if timestep == 0 {
        id1 = 0;
        id2 = 1;
    }else if timestep == timesteps {
        id1 = timesteps -1;
        id2 = timesteps
    }else
    {
        id1 = timestep;
        id2 = timestep +1;
    }
    let t1 = file
        .group(&format!("timestep {}",id1))
        .expect(&format!("Can not fine timestep {} in file {}",id1, file.filename()))
        .dataset("time")
        .expect(&format!(
            "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
            file.filename()
        ))
        .read_raw::<f64>()
        .expect(&format!(
            "Can not read data from \"time\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            file.filename()
        ))[0];
    let t2 = file
        .group(&format!("timestep {}",id2))
        .expect(&format!("Can not fine timestep {} in file {}",id2, file.filename()))
        .dataset("time")
        .expect(&format!(
            "Can not find dataset \"time\" in HDF5 file \"{:?}\"",
            file.filename()
        ))
        .read_raw::<f64>()
        .expect(&format!(
            "Can not read data from \"time\" dataset. \
            Data type or data format might be wrong. \
            Check creation of HDF5 file  \"{:?}\"",
            file.filename()
        ))[0];

    t2 - t1
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
    let mut partcloud: Array2<f64> = Array2::<f64>::zeros((buffersize,particles));
    for timestep in range[0]..range[1]{
        // for each particle find the Position at the
        let part_pos = file.group(&format!("timestep {}",timestep))
                        .expect(&format!("Can not find group \"timestep {}\" in file {}",
                            timestep,
                            file.filename()))
                        .dataset("position")
                        .expect(&format!("Can not find dataset \"position\" in file {}",
                            file.filename()))
                        .read_2d::<f64>()
                        .expect(&format!("Can not read dataset \"position\" in file {}",
                            file.filename())).slice(s![range[0]..range[1]+1,..]).to_owned();

        position.slice_mut(s![timestep,..,..])
                .assign(&part_pos);

        // for each particle find the velocuty at the
        let part_vel = file.group(&format!("timestep {}",timestep))
                        .expect(&format!("Can not find group \"timestep {}\" in file {}",
                            timestep,
                            file.filename()))
                        .dataset("velocity")
                        .expect(&format!("Can not find dataset \"velocity\" in file {}",
                            file.filename()))
                        .read_2d::<f64>()
                        .expect(&format!("Can not read dataset \"velocity\" in file {}",
                            file.filename())).slice(s![range[0]..range[1]+1,..]).to_owned();
        velocity.slice_mut(s![timestep,..,..])
                .assign(&part_vel);
        //Radius
        let part_rad = file.group(&format!("timestep {}",timestep))
            .expect(&format!("Can not fine timestep {} in file {}",timestep, file.filename()))
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
        radius.slice_mut(s![timestep,..])
                .assign(&array![ part_rad]);
                //Particle IDs
                let part_id = file.group(&format!("timestep {}",timestep))
                    .expect(&format!("Can not fine timestep {} in file {}",timestep, file.filename()))
                    .dataset("particleid")
                    .expect(&format!(
                        "Can not find dataset \"particleid\" in HDF5 file \"{:?}\"",
                        file.filename()
                    ))
                    .read_1d::<u64>()
                    .expect(&format!(
                        "Can not read data from \"particleid\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        file.filename()
                    )).mapv(|x| x as f64);
                particleid.slice_mut(s![timestep,..])
                        .assign(&part_id);
                //clouds
                let part_cloud = file.group(&format!("timestep {}",timestep))
                    .expect(&format!("Can not fine timestep {} in file {}",timestep, file.filename()))
                    .dataset("ppcloud")
                    .expect(&format!(
                        "Can not find dataset \"ppcloud\" in HDF5 file \"{:?}\"",
                        file.filename()
                    ))
                    .read_1d::<f64>()
                    .expect(&format!(
                        "Can not read data from \"ppcloud\" dataset. \
                        Data type or data format might be wrong. \
                        Check creation of HDF5 file  \"{:?}\"",
                        file.filename()
                    ));
                partcloud.slice_mut(s![timestep,..])
                        .assign(&part_cloud);
    }
    buffer.update("position",position.into_dyn(),range.clone());
    buffer.update("velocity",velocity.into_dyn(),range.clone());
    buffer.update("radius",radius.into_dyn(),range.clone());
    buffer.update("particleid",particleid.into_dyn(),range.clone());
    buffer.update("ppcloud",partcloud.into_dyn(),range.clone());
}
