This code evaluates the query efficiency of the transaction indexing mechanism in Zkfhed. 

We generated a batch of blockchain transactions containing CID and HP, and maintained both Ethereum's chain indexing and the new indexing structure locally. We simulated the process of a validator querying all workers' transactions from the chain before aggregating the global model and counted the total query time.