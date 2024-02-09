pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }

    stages {
        stage('Check') {
            steps {
                sh 'cargo check'
            }
        }
    }
}