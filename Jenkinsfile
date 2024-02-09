pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }

    environment {
        DEV = 'dummy'
        FAKE_NUMBER = 'dummy'
        TWILIO_NUMBER = 'dummy'
        TWILIO_ACCOUNT_ID = 'dummy'
        TWILIO_AUTH_TOKEN = 'dummy'
        DB_URI = 'dummy'
        DB_NAME = 'dummy'
    }

    stages {
        stage('Check') {
            steps {
                sh 'printenv | grep dummy > .env'
                sh 'cargo check'
            }
        }
    }
}