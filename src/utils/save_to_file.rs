use crate::DataRow;
use arrow::array::{BooleanArray, Float64Array, StringDictionaryBuilder, UInt32Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::path::Path;
use std::sync::Arc;

pub fn save_to_file(data: Vec<DataRow>) {
    use arrow::datatypes;

    // Save all the data to an Apache IPC file
    let schema = Arc::new(Schema::new(vec![
        Field::new("generation_id", DataType::UInt32, false),
        Field::new("shifted", DataType::Boolean, false),
        Field::new_dict(
            "distribution",
            DataType::Dictionary(Box::from(DataType::Int8), Box::from(DataType::Utf8)),
            false,
            0,
            false,
        ),
        Field::new_dict(
            "coordination_mechanism",
            DataType::Dictionary(Box::from(DataType::Int8), Box::from(DataType::Utf8)),
            false,
            1,
            false,
        ),
        Field::new_dict(
            "voting_mechanism",
            DataType::Dictionary(Box::from(DataType::Int8), Box::from(DataType::Utf8)),
            false,
            2,
            false,
        ),
        Field::new("number_of_proxies", DataType::UInt32, false),
        Field::new("number_of_delegators", DataType::UInt32, false),
        Field::new("estimate", DataType::Float64, false),
        Field::new("min_proxy_weight", DataType::Float64, false),
        Field::new("max_proxy_weight", DataType::Float64, false),
        Field::new("average_proxy_weight", DataType::Float64, false),
        Field::new("median_proxy_weight", DataType::Float64, false),
    ]));

    // Create the dictionary arrays
    let mut id_builder = UInt32Array::builder(data.len());
    let mut shifted_builder = BooleanArray::builder(data.len());

    let mut distribution_array_builder =
        StringDictionaryBuilder::<datatypes::Int8Type>::new();
    let mut coordination_mechanism_array_builder =
        StringDictionaryBuilder::<datatypes::Int8Type>::new();
    let mut voting_mechanism_array_builder =
        StringDictionaryBuilder::<datatypes::Int8Type>::new();

    let mut number_of_proxies_array_builder = UInt32Array::builder(data.len());
    let mut number_of_delegators_array_builder = UInt32Array::builder(data.len());

    let mut estimate_array_builder = Float64Array::builder(data.len());

    let mut min_proxy_weight_array_builder = Float64Array::builder(data.len());
    let mut max_proxy_weight_array_builder = Float64Array::builder(data.len());
    let mut average_proxy_weight_array_builder = Float64Array::builder(data.len());
    let mut median_proxy_weight_array_builder = Float64Array::builder(data.len());

    for row in data {
        id_builder.append_value(row.generation_id);
        shifted_builder.append_value(row.shifted);

        distribution_array_builder
            .append(row.distribution)
            .expect("Failed to append distribution");
        coordination_mechanism_array_builder
            .append(row.coordination_mechanism)
            .expect("Failed to append coordination mechanism");
        voting_mechanism_array_builder
            .append(row.voting_mechanism)
            .expect("Failed to append voting mechanism");

        number_of_proxies_array_builder.append_value(row.number_of_proxies);
        number_of_delegators_array_builder.append_value(row.number_of_delegators);

        estimate_array_builder.append_value(row.estimate);

        min_proxy_weight_array_builder.append_value(row.min_proxy_weight);
        max_proxy_weight_array_builder.append_value(row.max_proxy_weight);
        average_proxy_weight_array_builder.append_value(row.average_proxy_weight);
        median_proxy_weight_array_builder.append_value(row.median_proxy_weight);
    }

    let id_array = id_builder.finish();
    let shifted_array = shifted_builder.finish();

    let distribution_array = distribution_array_builder.finish();
    let coordination_mechanism_array = coordination_mechanism_array_builder.finish();
    let voting_mechanism_array = voting_mechanism_array_builder.finish();

    let number_of_proxies_array = number_of_proxies_array_builder.finish();
    let number_of_delegators_array = number_of_delegators_array_builder.finish();

    let estimate_array = estimate_array_builder.finish();

    let min_proxy_weight_array = min_proxy_weight_array_builder.finish();
    let max_proxy_weight_array = max_proxy_weight_array_builder.finish();
    let average_proxy_weight_array = average_proxy_weight_array_builder.finish();
    let median_proxy_weight_array = median_proxy_weight_array_builder.finish();

    let batch = RecordBatch::try_new(
        Arc::clone(&schema),
        vec![
            Arc::new(id_array),
            Arc::new(shifted_array),
            Arc::new(distribution_array),
            Arc::new(coordination_mechanism_array),
            Arc::new(voting_mechanism_array),
            Arc::new(number_of_proxies_array),
            Arc::new(number_of_delegators_array),
            Arc::new(estimate_array),
            Arc::new(min_proxy_weight_array),
            Arc::new(max_proxy_weight_array),
            Arc::new(average_proxy_weight_array),
            Arc::new(median_proxy_weight_array),
        ],
    )
    .expect("Error creating record batch");

    // Write the file
    if !Path::new("data/").exists() {
        std::fs::create_dir("data/").expect("Could not create directory");
    }

    let filepath = format!(
        "data/{}.arrow",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    );
    println!("Saving data to {filepath}");
    let mut writer = arrow::ipc::writer::FileWriter::try_new(
        std::fs::File::create(filepath).expect("Could not create file"),
        &schema,
    )
    .expect("Failed to create file writer");
    writer.write(&batch).expect("Failed to write batch");

    writer.finish().expect("Error finishing file writer");
}
