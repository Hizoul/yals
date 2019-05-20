pipeline {
  agent none
  stages {
    stage('Windows Build') {
      agent {
        docker { image 'tomaka/rust-mingw-docker' }
      }
      steps {
        sh 'rustup default stable-x86_86-pc-windows-gnu'
        sh 'rustup target add x86_86-pc-windows-gnu'
        sh 'rustup target add i686-pc-windows-gnu'
        sh 'sh scripts/build_windows.sh'
        archiveArtifacts artifacts: 'target/x86_86-pc-windows-gnu/release/yals.exe', fingerprint: true
        archiveArtifacts artifacts: 'target/i686-pc-windows-gnu/release/yals.exe', fingerprint: true
      }
    }
    stage('Linux Build') {
      agent {
        docker { image 'ekidd/rust-musl-builder' }
      }
      steps {
        sh 'rustup default stable'
        sh 'rustup target add x86_64-unknown-linux-musl'
        sh 'rustup target add i686-unknown-linux-musl'
        sh 'sh scripts/build_linux.sh'
        archiveArtifacts artifacts: 'target/x86_64-unknown-linux-musl/release/yals', fingerprint: true
        archiveArtifacts artifacts: 'target/i686-unknown-linux-musl/release/yals', fingerprint: true 
      }
    }
  }
}