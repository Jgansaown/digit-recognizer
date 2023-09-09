type ModelTypes = KMeansType | KNearestNeighborsType | PerceptronType;

interface ModelParameters {
    kmeans: KMeansParam;
    knn: KNearestNeighborsParam;
    perceptron: PerceptronParam;
}

type ModelParametersUnion = {
    [K in keyof ModelParameters]: { type: K; param: ModelParameters[K] };
}[keyof ModelParameters];
