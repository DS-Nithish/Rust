const { MongoClient } = require('mongodb');
const uri = "mongodb://localhost:27017";
const client = new MongoClient(uri);
const dbName = 'test';
const collectionName = 'msc';

async function run() {
    try {
        await client.connect();
        console.log("Connected to MongoDB");
        const db = client.db(dbName);
        const collection = db.collection(collectionName);

        // 1. Insert a document
        const insertResult = await collection.insertOne({
            _id: '12',
            name: "Devi",
            age: 22,
            place: "Chennai"
        });
        console.log("Inserted document ID:",  insertResult.insertedId);

        // 2. Find documents
        const students = await collection.find({}).toArray();
        console.log("Found documents:", students);

        // 3. Update a document
        const UpdateResult = await collection.updateOne(
            { name: "Devi" },
            { $set: { place: "Cbe" } }
        );
        console.log("Updated document count:", UpdateResult.modifiedCount);

        



// 4. Delete a document
        const deleteResult = await collection.deleteOne({ name: "Jana" });
        console.log("Deleted document count:", deleteResult.deleteCount);

    } catch (err) {
        console.error("An error occurred", err);
    } finally {
        // Close the connection
        await client.close();
        console.log("Connection to MongoDB closed");
    }
}

run();

