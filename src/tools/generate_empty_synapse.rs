use synapse::asg::ASG;
use synapse::syn1_writer::save_synapse_file;
use anyhow::Result;

fn main() -> Result<()> {
    let asg = ASG {
        nodes: vec![],
        entry: 0,
    };

    save_synapse_file("empty.synapse", &asg)?;
    println!("Empty .synapse file 'empty.synapse' created successfully.");
    Ok(())
}
