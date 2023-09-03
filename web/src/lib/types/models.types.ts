type ModelTypes = KMeansType | KNearestNeighborsType | PerceptronType;

interface ModelParameters {
    kmeans: KMeansParam;
    knn: KNearestNeighborsParam;
    perceptron: PerceptronParam;
}

function test<T extends ModelTypes>(type: T, param: ModelParameters[T]): ModelParameters[T] {
    return param;
}
